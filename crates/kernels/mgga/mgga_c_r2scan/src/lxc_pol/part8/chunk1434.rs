//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1434/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1434<F: Float>(t3229: F, t7217: F, t1556: F, t1562: F, t23041: F, t2530: F, t2531: F, t2534: F, t2538: F, t2847: F, t3052: F, t3053: F, t3056: F, t3060: F, t31268: F, t32330: F, t32381: F, t32419: F, t32471: F, t32514: F, t32554: F, t32596: F, t32636: F, t32663: F, t32692: F, t32716: F, t32765: F, t32809: F, t32846: F, t32870: F, t32918: F, t33068: F, t33084: F, t33100: F, t33128: F, t33152: F, t33182: F, t33226: F, t33256: F, t33285: F, t33309: F, t33345: F, t33377: F, t33394: F, t33812: F, t33849: F, t33876: F, t33903: F, t33938: F, t33953: F, t33982: F, t34019: F, t34042: F, t34067: F, t34089: F, t34112: F, t34129: F, t34187: F, t34210: F, t34238: F, t34258: F, t34290: F, t34334: F, t34373: F, t34387: F, t34410: F, t34431: F, t34443: F, t34460: F, t34474: F, t34504: F, t34532: F, t34570: F, t34596: F, t34628: F, t34650: F, t34682: F, t34715: F, t34735: F, t494: F, t495: F, t496: F, t499: F, t5087: F, t7206: F, t7221: F, t792: F, t8692: F, t8694: F, t8698: F, t8707: F, t8714: F, t921: F, t9560: F, t983: F, t9948: F, t9950: F, t9955: F, t9964: F) -> (F,) {
    let t34768 = t7217 * t3229;
    let t34771 = 3.0 / 4.0 * t921 * t31268 - 15.0 / 16.0 * t1562 * t983 * t9560 + 3.0 / 4.0 * t3056 * t7221 + t9948 * t1556 / 4.0 + 3.0 * t3056 * t8698 + 3.0 / 4.0 * t3053 * t7221 + t499 * (t34210 + t32870 + t32554 + t33982 + t32846 + t33876 + t33812 + t33309 + t34715 + t33849 + t33152 + t32716 + t34042 + t33226 + t32596 + t33068 + t34628 + t34460 + t32663 + t34474 + t34238 + t34387 + t34570 + t34290 + t34410 + t32809 + t34089 + t34532 + t34258 + t34187 + t34067 + t33377 + t34650 + t32918 + t32514 + t34735 + t33084 + t33938 + t33394 + t33345 + t33182 + t34596 + t32381 + t33256 + t32765 + t33903 + t34112 + t32471 + t33953 + t34334 + t34431 + t34682 + t32330 + t34373 + t34443 + t34019 + t32636 + t33128 + t34504 + t32419 + t33285 + t32692 + t33100 + t34129) / 4.0 + 3.0 * t3052 * t2530 * t2534 + t9955 * t494 * t2534 - 15.0 / 16.0 * t2531 * t8714 - 585.0 / 256.0 * t23041 * t9964 * t792 + 3.0 / 2.0 * t921 * t2538 * t2530 + 135.0 / 64.0 * t5087 * t3060 * t2847 + 3.0 / 4.0 * t9950 * t7206 + 3.0 / 4.0 * t8692 * t2538 + 3.0 * t9950 * t496 + 3.0 / 4.0 * t8694 * t8707 - 15.0 / 16.0 * t495 * t34768;
    (t34771,)
}
