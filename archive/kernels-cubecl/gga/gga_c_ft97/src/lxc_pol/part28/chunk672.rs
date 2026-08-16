//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 672/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk672<F: Float>(t1355: F, t23847: F, t23869: F, t23874: F, t23877: F, t26661: F, t26665: F, t26678: F, t26729: F, t26740: F, t26745: F, t26751: F, t26759: F, t26763: F, t5785: F, t5802: F, t5813: F, t6593: F, t8833: F, t8838: F) -> F {
    let t26765 = F::cast_from(0.45306850413028723348e0_f64) * t26729 * t6593 + F::cast_from(0.45306850413028723348e0_f64) * t5785 * t26678 + F::cast_from(0.24163653553615319119e1_f64) * t5802 * t26665 - F::cast_from(0.12081826776807659559e1_f64) * t1355 * t26661 - F::cast_from(0.45306850413028723348e0_f64) * t23847 * t26740 + F::cast_from(0.45306850413028723348e0_f64) * t23869 * t26745 + F::cast_from(0.45306850413028723348e0_f64) * t23869 * t26740 - F::cast_from(0.45306850413028723348e0_f64) * t8833 * t26751 + F::cast_from(0.45306850413028723348e0_f64) * t8838 * t26751 - F::cast_from(0.45306850413028723348e0_f64) * t23847 * t26745 + F::cast_from(0.44452000728395061731e-1_f64) * t23874 + t23877 + F::cast_from(0.26671200437037037037e0_f64) * t5813 * t26759 - F::cast_from(0.33339000546296296297e-1_f64) * t26763;
    t26765
}
