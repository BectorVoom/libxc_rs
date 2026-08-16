//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1616/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616<F: Float>(t13039: F, t44372: F, t44373: F, t13045: F, t42871: F, t3597: F, t3603: F, t3367: F, t2251: F, t12839: F, t2258: F, t1042: F, t1261: F, t12784: F, t12803: F, t12810: F, t12836: F, t12842: F, t13100: F, t17426: F, t17638: F, t17644: F, t247: F, t3610: F, t3611: F, t3625: F, t3626: F, t3629: F, t3674: F, t43777: F, t44333: F, t44377: F, t44418: F, t44422: F, t44427: F, t44431: F, t5340: F) -> F {
    let t44441 = t44372 * t13039 * t44373;
    let t44442 = t42871 * t13045;
    let t44448 = t44372 * t3597 * t44373;
    let t44449 = t42871 * t3603;
    let t44458 = t3603 * t3367;
    let t44459 = t44458 * t2251;
    let t44466 = t12839 * t2258;
    let t44479 = -F::cast_from(0.11433071498151929859e-2_f64) * t44418 + F::cast_from(0.25724410870841842184e-2_f64) * t44422 * t3674 + F::cast_from(0.19055119163586549765e-2_f64) * t44427 - F::cast_from(0.17149607247227894789e-2_f64) * t12784 * t12836 - F::cast_from(0.57165357490759649296e-3_f64) * t3625 * t3626 * t44431 * t3629 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t12803 * t17638 - F::cast_from(0.77173232612525526552e-2_f64) * t44441 * t1042 * t44377 * t44442 + F::cast_from(0.30011812682648815881e-2_f64) * t44448 * t1042 * t44377 * t44449 - F::cast_from(0.64311027177104605458e-3_f64) * t3610 * t1042 * t44333 * t3611 - F::cast_from(0.34299214494455789578e-2_f64) * t5340 * t3626 * t12810 * t44459 - F::cast_from(0.34299214494455789578e-2_f64) * t17426 * t12842 - F::cast_from(0.17149607247227894789e-2_f64) * t5340 * t3626 * t12810 * t44466 - F::cast_from(0.17149607247227894789e-2_f64) * t3625 * t3626 * t12803 * t17644 - F::cast_from(0.76220476654346199062e-2_f64) * t1261 * t247 * t13100 * t43777;
    t44479
}
