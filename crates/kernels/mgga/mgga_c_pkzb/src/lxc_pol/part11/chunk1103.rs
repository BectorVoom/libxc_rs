//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1103/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1103<F: Float>(t11020: F, t17848: F, t2104: F, t2030: F, t9539: F, t10993: F, t11021: F, t11030: F, t11037: F, t2031: F, t2105: F, t21624: F, t21627: F, t21652: F, t21658: F, t21661: F, t25221: F, t25434: F, t25448: F, t25453: F, t25456: F, t2739: F, t2899: F, t2900: F, t2922: F, t302: F, t3645: F, t3679: F, t5984: F, t761: F, t7653: F, t7700: F, t7701: F, t7725: F, t9194: F, t9258: F, t9263: F, t9268: F, t9691: F) -> (F, F) {
    let t29877 = t2104 * t17848 * t11020;
    let t29894 = t2030 * t9539;
    let t29905 = -0.25724410870841842184e-2 * t2899 * t25221 * t9263 + 0.38586616306262763276e-2 * t2104 * t9258 * t761 * t9194 - 0.25724410870841842184e-2 * t2899 * t7700 * t2031 * t9268 + 0.12862205435420921092e-2 * t2922 * t7700 * t7701 * t11037 + t21624 + t21627 - t21652 - t21658 - 0.38110238327173099531e-3 * t21661 + 0.25724410870841842183e-2 * t29877 + 0.34299214494455789578e-2 * t7725 * t11030 - 0.12862205435420921092e-2 * t2104 * t2105 * t9691 * t3645 - 0.25724410870841842183e-2 * t2899 * t2105 * t3679 * t2030 * t2739 + 0.12862205435420921092e-2 * t2899 * t302 * t7653 * t10993 + 0.12862205435420921092e-2 * t2899 * t302 * t2900 * t29894 + 0.17149607247227894789e-2 * t25434 - 0.20579528696673473747e-1 * t5984 * t11021 + 0.85748036236139473944e-3 * t25448 - 0.17149607247227894789e-2 * t25453 + 0.17149607247227894789e-2 * t25456;
    (t29894, t29905)
}
