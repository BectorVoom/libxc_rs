//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 672/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk672(t1355: f64, t23847: f64, t23869: f64, t23874: f64, t23877: f64, t26661: f64, t26665: f64, t26678: f64, t26729: f64, t26740: f64, t26745: f64, t26751: f64, t26759: f64, t26763: f64, t5785: f64, t5802: f64, t5813: f64, t6593: f64, t8833: f64, t8838: f64) -> f64 {
    let t26765 = 0.45306850413028723348e0_f64 * t26729 * t6593 + 0.45306850413028723348e0_f64 * t5785 * t26678 + 0.24163653553615319119e1_f64 * t5802 * t26665 - 0.12081826776807659559e1_f64 * t1355 * t26661 - 0.45306850413028723348e0_f64 * t23847 * t26740 + 0.45306850413028723348e0_f64 * t23869 * t26745 + 0.45306850413028723348e0_f64 * t23869 * t26740 - 0.45306850413028723348e0_f64 * t8833 * t26751 + 0.45306850413028723348e0_f64 * t8838 * t26751 - 0.45306850413028723348e0_f64 * t23847 * t26745 + 0.44452000728395061731e-1_f64 * t23874 + t23877 + 0.26671200437037037037e0_f64 * t5813 * t26759 - 0.33339000546296296297e-1_f64 * t26763;
    t26765
}
