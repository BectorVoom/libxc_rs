//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 987/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk987(t43486: f64, t7427: f64, t7573: f64, t33294: f64, t9839: f64, t10062: f64, t3040: f64, t3295: f64, t8802: f64, t9800: f64, t13052: f64, t1966: f64) -> (f64, f64, f64, f64, f64) {
    let t43750 = 0.12423108009070322895e3_f64 * t7427 * t7573 * t43486;
    let t43752 = 0.47667319935800568892e0_f64 * t33294 * t9839;
    let t43754 = 0.35750489951850426669e0_f64 * t10062 * t3040;
    let t43756 = t9800 * t8802 * t3295;
    let t43757 = 0.19171462976960374838e1_f64 * t43756;
    let t43758 = t1966 * t13052;
    (t43750, t43752, t43754, t43757, t43758)
}
