//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 777/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk777(t225: f64, t7085: f64, t10110: f64, t2053: f64, t2719: f64, t23251: f64, t23261: f64, t7106: f64, t865: f64, t2718: f64, t2742: f64, t10049: f64, t2054: f64, t23243: f64, t23249: f64, t23254: f64, t23259: f64, t23266: f64, t23274: f64, t2597: f64, t2713: f64, t2743: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t866: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64) {
    let t24305 = t7085 * t225;
    let t24314 = t10110 * t2053 * t2719;
    let t24318 = 0.52089578783527170489e-1_f64 * t23251;
    let t24321 = 0.12793931631041761173e0_f64 * t23261;
    let t24324 = t7106 * t865;
    let t24325 = t2718 * t24324;
    let t24330 = t2718 * t2053 * t2742;
    let t24333 = -2.0_f64 * t2597 * t7107 + 0.9869604401089358619e-1_f64 * t23243 - t9590 * t2054 - 2.0_f64 * t24305 * t866 - t10049 * t2054 - 2.0_f64 * t9593 * t2054 + 4.0_f64 * t2713 * t7092 - 6.0_f64 * t855 * t24314 - 0.76763589786250567036e-1_f64 * t23249 + t24318 - 0.16449340668482264365e-1_f64 * t23254 + 0.16449340668482264365e-1_f64 * t23259 + t24321 - 0.3289868133696452873e-1_f64 * t23266 - t7087 * t2743 + 4.0_f64 * t855 * t24325 + 0.6579736267392905746e-1_f64 * t23274 + 2.0_f64 * t855 * t24330;
    (t24305, t24314, t24325, t24330, t24333)
}
