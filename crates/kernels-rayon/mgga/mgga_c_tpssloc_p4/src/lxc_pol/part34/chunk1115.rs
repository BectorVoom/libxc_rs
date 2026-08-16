//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1115/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1115(t12248: f64, t2085: f64, t81317: f64, t81398: f64, t2056: f64, t40772: f64, t82069: f64, t81598: f64, t81735: f64, t81742: f64, t81849: f64, t81852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84627 = t12248 * t2085;
    let t84659 = 0.55440370401180965083e0_f64 * t81317;
    let t84705 = 0.27415567780803773942e-2_f64 * t81398;
    let t84766 = t2056 * t40772;
    let t84820 = 0.19739208802178717238e0_f64 * t82069;
    let t84851 = 0.3244175520728446583e0_f64 * t81598;
    let t84857 = 0.13958506597733353653e-1_f64 * t81735;
    let t84859 = 0.87474304870637513515e-3_f64 * t81742;
    let t84896 = 0.2034786907144675699e0_f64 * t81849;
    let t84897 = 455.0_f64 / 648.0_f64 * t81852;
    (t84627, t84659, t84705, t84766, t84820, t84851, t84857, t84859, t84896, t84897)
}
