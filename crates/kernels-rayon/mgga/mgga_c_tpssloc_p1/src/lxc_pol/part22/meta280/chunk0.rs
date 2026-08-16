//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1429/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1429(t12923: f64, t607: f64, t4194: f64, t3966: f64, t751: f64, t707: f64, t157: f64, t9897: f64, t2371: f64, t4199: f64, t1409: f64, t2517: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12924 = t12923 * t607;
    let t12926 = 24.0_f64 * t4194 * t12924;
    let t12932 = t751 * t3966;
    let t12934 = 8.0_f64 * t707 * t12932;
    let t12939 = t9897 * t157;
    let t12943 = t4199 * t2371;
    let t12945 = t2517 * t1409;
    (t12924, t12926, t12932, t12934, t12939, t12943, t12945)
}
