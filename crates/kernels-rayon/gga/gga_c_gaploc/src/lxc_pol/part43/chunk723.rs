//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 723/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk723(t12769: f64, t12799: f64, t12802: f64, t12805: f64, t12809: f64, t12812: f64, t13726: f64, t13730: f64, t13733: f64, t13741: f64, t13753: f64, t12823: f64, t12824: f64, t12825: f64, t12828: f64, t12829: f64, t12832: f64, t12833: f64, t12836: f64, t12842: f64, t13736: f64, t13758: f64) -> (f64, f64) {
    let t14450 = t12769 - 0.23712505529730124666e-2_f64 * t13741 + 0.1138200265427045984e0_f64 * t13730 + t12799 - t12802 + t12805 + 0.23712505529730124666e-2_f64 * t13726 - 0.17073003981405689759e0_f64 * t13733 - t12809 - t13753 + t12812;
    let t14452 = 0.56910013271352299198e-1_f64 * t13736 + t13758 - t12823 + t12824 + t12825 + t12828 + t12829 - t12832 - t12833 + t12836 - t12842;
    (t14450, t14452)
}
