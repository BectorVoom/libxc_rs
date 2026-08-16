//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3125/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125(t343: f64, t56: f64, t816: f64, t13026: f64, t65: f64, t12256: f64, t1121: f64, t1222: f64, t1250: f64, t12797: f64, t12866: f64, t13102: f64, t17353: f64, t17426: f64, t17475: f64, t17672: f64, t17705: f64, t17747: f64, t17748: f64, t247: f64, t3584: f64, t3625: f64, t3626: f64, t3718: f64, t3719: f64, t3720: f64, t44548: f64, t44559: f64, t44571: f64, t44583: f64, t5056: f64, t51959: f64, t5312: f64, t5373: f64, t5381: f64, t56149: f64, t56201: f64, t56219: f64, t56561: f64, t57498: f64, t57508: f64, t57520: f64, t57534: f64, t57536: f64, t606: f64) -> (f64, f64) {
    let t57548 = t56 * t343 * t816;
    let t57549 = t65 * t13026;
    let t57550 = t57549 * t12256;
    let t57555 = -0.64311027177104605458e-3_f64 * t3718 * t3720 * t57498 * t1250 + 0.28582678745379824648e-3_f64 * t44548 - 0.63517063878621832552e-3_f64 * t5381 * t13102 - 0.25724410870841842184e-2_f64 * t57508 + 0.85748036236139473944e-3_f64 * t44559 + t1222 * t5312 * t56201 / 12.0_f64 + t1222 * t5312 * t56149 / 6.0_f64 - 7.0_f64 / 216.0_f64 * t1222 * t17475 * t56219 + 0.51448821741683684368e-2_f64 * t57520 * t247 * t3719 * t56561 - t5373 * t12797 / 27.0_f64 + 0.42874018118069736972e-3_f64 * t12866 * t17353 * t1250 * t3584 * t1121 * t606 + 0.47637797908966374413e-3_f64 * t44571 + 0.17149607247227894789e-2_f64 * t57534 - 0.38586616306262763275e-2_f64 * t17747 * t3720 * t57536 * t17748 + 0.12862205435420921092e-2_f64 * t17426 * t17705 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t5056 * t17672 - 7.0_f64 / 216.0_f64 * t57548 * t57550 * t51959 - 0.85748036236139473944e-3_f64 * t44583;
    (t57548, t57555)
}
