//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1616/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616(t13039: f64, t44372: f64, t44373: f64, t13045: f64, t42871: f64, t3597: f64, t3603: f64, t3367: f64, t2251: f64, t12839: f64, t2258: f64, t1042: f64, t1261: f64, t12784: f64, t12803: f64, t12810: f64, t12836: f64, t12842: f64, t13100: f64, t17426: f64, t17638: f64, t17644: f64, t247: f64, t3610: f64, t3611: f64, t3625: f64, t3626: f64, t3629: f64, t3674: f64, t43777: f64, t44333: f64, t44377: f64, t44418: f64, t44422: f64, t44427: f64, t44431: f64, t5340: f64) -> f64 {
    let t44441 = t44372 * t13039 * t44373;
    let t44442 = t42871 * t13045;
    let t44448 = t44372 * t3597 * t44373;
    let t44449 = t42871 * t3603;
    let t44458 = t3603 * t3367;
    let t44459 = t44458 * t2251;
    let t44466 = t12839 * t2258;
    let t44479 = -0.11433071498151929859e-2_f64 * t44418 + 0.25724410870841842184e-2_f64 * t44422 * t3674 + 0.19055119163586549765e-2_f64 * t44427 - 0.17149607247227894789e-2_f64 * t12784 * t12836 - 0.57165357490759649296e-3_f64 * t3625 * t3626 * t44431 * t3629 - 0.85748036236139473944e-3_f64 * t3625 * t3626 * t12803 * t17638 - 0.77173232612525526552e-2_f64 * t44441 * t1042 * t44377 * t44442 + 0.30011812682648815881e-2_f64 * t44448 * t1042 * t44377 * t44449 - 0.64311027177104605458e-3_f64 * t3610 * t1042 * t44333 * t3611 - 0.34299214494455789578e-2_f64 * t5340 * t3626 * t12810 * t44459 - 0.34299214494455789578e-2_f64 * t17426 * t12842 - 0.17149607247227894789e-2_f64 * t5340 * t3626 * t12810 * t44466 - 0.17149607247227894789e-2_f64 * t3625 * t3626 * t12803 * t17644 - 0.76220476654346199062e-2_f64 * t1261 * t247 * t13100 * t43777;
    t44479
}
