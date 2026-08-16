//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 668/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk668<F: Float>(t1701: F, t3379: F, t5546: F, t1355: F, t2043: F, t22767: F, t23766: F, t23770: F, t23781: F, t23810: F, t23842: F, t26613: F, t26617: F, t26621: F, t26650: F, t26658: F, t26661: F, t26665: F, t26671: F, t26674: F, t5579: F, t5785: F, t5802: F, t5829: F, t6593: F, t6608: F, t8852: F) -> (F, F) {
    let t26678 = t1701 * t5546 * t3379;
    let t26686 = -F::cast_from(0.24167761770734866964e0_f64) * t23842 * t26613 + F::cast_from(0.10001700163888888889e0_f64) * t5829 * t5579 * t26650 - F::cast_from(0.26671200437037037037e0_f64) * t5829 * t22767 * t6608 + F::cast_from(0.33339000546296296297e-1_f64) * t26658 + F::cast_from(0.12081826776807659559e1_f64) * t2043 * t26661 - F::cast_from(0.24163653553615319119e1_f64) * t5785 * t26665 + F::cast_from(0.22653425206514361674e0_f64) * t1355 * t26621 + F::cast_from(0.54738951849294959987e0_f64) * t8852 * t26671 - F::cast_from(0.45306850413028723348e0_f64) * t26674 * t6593 - F::cast_from(0.45306850413028723348e0_f64) * t5802 * t26678 - F::cast_from(0.10947790369858991997e1_f64) * t23810 * t26617 + F::cast_from(0.33339000546296296298e-1_f64) * t23766 - F::cast_from(0.40279602951224778275e-1_f64) * t23770 + F::cast_from(0.40279602951224778275e-1_f64) * t23781;
    (t26678, t26686)
}
