//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1216/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1216(t11020: f64, t17848: f64, t2104: f64, t2030: f64, t9539: f64, t10993: f64, t11021: f64, t11030: f64, t11037: f64, t2031: f64, t2105: f64, t21624: f64, t21627: f64, t21652: f64, t21658: f64, t21661: f64, t25221: f64, t25434: f64, t25448: f64, t25453: f64, t25456: f64, t2739: f64, t2899: f64, t2900: f64, t2922: f64, t302: f64, t3645: f64, t3679: f64, t5984: f64, t761: f64, t7653: f64, t7700: f64, t7701: f64, t7725: f64, t9194: f64, t9258: f64, t9263: f64, t9268: f64, t9691: f64) -> (f64, f64) {
    let t29877 = t2104 * t17848 * t11020;
    let t29894 = t2030 * t9539;
    let t29905 = -0.25724410870841842184e-2_f64 * t2899 * t25221 * t9263 + 0.38586616306262763276e-2_f64 * t2104 * t9258 * t761 * t9194 - 0.25724410870841842184e-2_f64 * t2899 * t7700 * t2031 * t9268 + 0.12862205435420921092e-2_f64 * t2922 * t7700 * t7701 * t11037 + t21624 + t21627 - t21652 - t21658 - 0.38110238327173099531e-3_f64 * t21661 + 0.25724410870841842183e-2_f64 * t29877 + 0.34299214494455789578e-2_f64 * t7725 * t11030 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t9691 * t3645 - 0.25724410870841842183e-2_f64 * t2899 * t2105 * t3679 * t2030 * t2739 + 0.12862205435420921092e-2_f64 * t2899 * t302 * t7653 * t10993 + 0.12862205435420921092e-2_f64 * t2899 * t302 * t2900 * t29894 + 0.17149607247227894789e-2_f64 * t25434 - 0.20579528696673473747e-1_f64 * t5984 * t11021 + 0.85748036236139473944e-3_f64 * t25448 - 0.17149607247227894789e-2_f64 * t25453 + 0.17149607247227894789e-2_f64 * t25456;
    (t29894, t29905)
}
