//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1784/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1784(t81598: f64, t81735: f64, t81742: f64, t81849: f64, t81852: f64, t81920: f64, t81954: f64, t2627: f64, t7084: f64, t81688: f64, t81716: f64, t82046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84851 = 0.3244175520728446583e0_f64 * t81598;
    let t84857 = 0.13958506597733353653e-1_f64 * t81735;
    let t84859 = 0.87474304870637513515e-3_f64 * t81742;
    let t84896 = 0.2034786907144675699e0_f64 * t81849;
    let t84897 = 455.0_f64 / 648.0_f64 * t81852;
    let t84921 = 595.0_f64 / 2592.0_f64 * t81920;
    let t84932 = 0.67287926823567318088e-4_f64 * t81954;
    let t84962 = t2627 * t7084;
    let t84995 = 0.27415567780803773942e-2_f64 * t81688;
    let t85003 = 0.19739208802178717238e0_f64 * t81716;
    let t85027 = 0.55440370401180965083e0_f64 * t82046;
    (t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84962, t84995, t85003, t85027)
}
