//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1334/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1334<F: Float>(t2887: F, t68: F, t9301: F, t18021: F, t18236: F, t2031: F, t2104: F, t2105: F, t2106: F, t21946: F, t21950: F, t26494: F, t26537: F, t2899: F, t2901: F, t2922: F, t302: F, t3657: F, t3685: F, t6022: F, t7395: F, t761: F, t7648: F, t7671: F, t7676: F, t7700: F, t7701: F, t7736: F, t7737: F, t7742: F, t7743: F, t9258: F, t9562: F, t9575: F, t9691: F) -> (F,) {
    let t26592 = t2887 * t68 * t9301;
    let t26639 = t26592 / 36.0 + t21946 / 27.0 + t21950 / 108.0 - 0.10162730220579493208e-2 * t18236 - 0.72409452821628889107e-2 * t18021 * t3657 - 0.17149607247227894789e-2 * t2899 * t7700 * t2031 * t7676 + 0.51448821741683684366e-2 * t2104 * t9258 * t761 * t7395 - 0.34299214494455789578e-2 * t2899 * t7700 * t2031 * t7671 + 0.17149607247227894789e-2 * t2922 * t7700 * t7701 * t9575 - 0.85748036236139473944e-3 * t2104 * t2105 * t9691 * t2106 - 0.42874018118069736972e-3 * t2104 * t2105 * t3685 * t7648 + 0.85748036236139473944e-3 * t2899 * t302 * t26494 * t2901 + 0.42874018118069736972e-3 * t2899 * t302 * t9562 * t6022 + 0.12862205435420921092e-2 * t7736 * t302 * t26537 * t7737 - 0.12862205435420921092e-2 * t7742 * t302 * t26537 * t7743;
    (t26639,)
}
