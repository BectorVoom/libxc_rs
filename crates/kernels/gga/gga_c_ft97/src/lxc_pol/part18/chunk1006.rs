//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1006/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1006<F: Float>(t26738: F, t538: F, t22591: F, t1008: F, t58: F, t554: F, t22767: F, t6604: F, t22632: F, t5813: F, t1355: F, t23847: F, t23869: F, t23874: F, t23877: F, t26661: F, t26665: F, t26678: F, t26729: F, t5785: F, t5802: F, t6593: F, t8833: F, t8838: F) -> (F, F, F, F, F) {
    let t26739 = t26738 * t538;
    let t26740 = t22591 * t26739;
    let t26743 = t58 * t1008;
    let t26744 = t26743 * t554;
    let t26745 = t22591 * t26744;
    let t26750 = t26738 * t554;
    let t26751 = t22591 * t26750;
    let t26759 = t22767 * t6604;
    let t26762 = t22632 * t6604;
    let t26763 = t5813 * t26762;
    let t26765 = 0.45306850413028723348e0 * t26729 * t6593 + 0.45306850413028723348e0 * t5785 * t26678 + 0.24163653553615319119e1 * t5802 * t26665 - 0.12081826776807659559e1 * t1355 * t26661 - 0.45306850413028723348e0 * t23847 * t26740 + 0.45306850413028723348e0 * t23869 * t26745 + 0.45306850413028723348e0 * t23869 * t26740 - 0.45306850413028723348e0 * t8833 * t26751 + 0.45306850413028723348e0 * t8838 * t26751 - 0.45306850413028723348e0 * t23847 * t26745 + 0.44452000728395061731e-1 * t23874 + t23877 + 0.26671200437037037037e0 * t5813 * t26759 - 0.33339000546296296297e-1 * t26763;
    (t26743, t26745, t26759, t26762, t26765)
}
