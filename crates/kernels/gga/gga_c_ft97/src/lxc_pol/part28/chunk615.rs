//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 615/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk615<F: Float>(t3404: F, t72: F, t22632: F, t5829: F, t6608: F, t1013: F, t1701: F, t22652: F, t1008: F, t2035: F, t5790: F, t3347: F, t5784: F, t3379: F, t5546: F, t1355: F, t2043: F, t22767: F, t23766: F, t23770: F, t23781: F, t23810: F, t23842: F, t26613: F, t26617: F, t26621: F, t5579: F, t5785: F, t5802: F, t6593: F, t8852: F) -> (F, F, F, F, F) {
    let t26650 = t72 * t3404;
    let t26658 = t5829 * t22632 * t6608;
    let t26661 = t1701 * t22652 * t1013;
    let t26665 = t1701 * t22652 * t1008;
    let t26671 = t2035 * t5790 * t1013;
    let t26674 = t3347 * t5784;
    let t26678 = t1701 * t5546 * t3379;
    let t26686 = -0.24167761770734866964e0 * t23842 * t26613 + 0.10001700163888888889e0 * t5829 * t5579 * t26650 - 0.26671200437037037037e0 * t5829 * t22767 * t6608 + 0.33339000546296296297e-1 * t26658 + 0.12081826776807659559e1 * t2043 * t26661 - 0.24163653553615319119e1 * t5785 * t26665 + 0.22653425206514361674e0 * t1355 * t26621 + 0.54738951849294959987e0 * t8852 * t26671 - 0.45306850413028723348e0 * t26674 * t6593 - 0.45306850413028723348e0 * t5802 * t26678 - 0.10947790369858991997e1 * t23810 * t26617 + 0.33339000546296296298e-1 * t23766 - 0.40279602951224778275e-1 * t23770 + 0.40279602951224778275e-1 * t23781;
    (t26661, t26665, t26671, t26678, t26686)
}
