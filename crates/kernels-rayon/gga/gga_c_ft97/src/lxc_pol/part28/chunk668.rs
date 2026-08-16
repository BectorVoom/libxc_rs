//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 668/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk668(t1701: f64, t3379: f64, t5546: f64, t1355: f64, t2043: f64, t22767: f64, t23766: f64, t23770: f64, t23781: f64, t23810: f64, t23842: f64, t26613: f64, t26617: f64, t26621: f64, t26650: f64, t26658: f64, t26661: f64, t26665: f64, t26671: f64, t26674: f64, t5579: f64, t5785: f64, t5802: f64, t5829: f64, t6593: f64, t6608: f64, t8852: f64) -> (f64, f64) {
    let t26678 = t1701 * t5546 * t3379;
    let t26686 = -0.24167761770734866964e0_f64 * t23842 * t26613 + 0.10001700163888888889e0_f64 * t5829 * t5579 * t26650 - 0.26671200437037037037e0_f64 * t5829 * t22767 * t6608 + 0.33339000546296296297e-1_f64 * t26658 + 0.12081826776807659559e1_f64 * t2043 * t26661 - 0.24163653553615319119e1_f64 * t5785 * t26665 + 0.22653425206514361674e0_f64 * t1355 * t26621 + 0.54738951849294959987e0_f64 * t8852 * t26671 - 0.45306850413028723348e0_f64 * t26674 * t6593 - 0.45306850413028723348e0_f64 * t5802 * t26678 - 0.10947790369858991997e1_f64 * t23810 * t26617 + 0.33339000546296296298e-1_f64 * t23766 - 0.40279602951224778275e-1_f64 * t23770 + 0.40279602951224778275e-1_f64 * t23781;
    (t26678, t26686)
}
