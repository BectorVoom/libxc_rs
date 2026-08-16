//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1149/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1149(t34012: f64, t4299: f64, t10688: f64, t36068: f64, t2843: f64, t6386: f64, t7124: f64, t36042: f64, t875: f64, t34333: f64, t6963: f64, t10492: f64, t10703: f64, t112680: f64, t112920: f64, t11593: f64, t143604: f64, t143606: f64, t143608: f64, t143610: f64, t15229: f64, t152717: f64, t152815: f64, t152888: f64, t15290: f64, t15312: f64, t15460: f64, t1901: f64, t24886: f64, t28501: f64, t28760: f64, t2881: f64, t28847: f64, t28854: f64, t28930: f64, t29055: f64, t29071: f64, t29123: f64, t29150: f64, t33978: f64, t34081: f64, t34207: f64, t36164: f64, t3746: f64, t4260: f64, t44030: f64, t56418: f64, t6273: f64, t684: f64, t7629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t153715 = t34012 * t4299;
    let t153717 = t10688 * t36068;
    let t153720 = t2843 * t6386 * t7124;
    let t153723 = t2843 * t36042 * t875;
    let t153725 = t6963 * t34333;
    let t153788 = 2.0_f64 / 3.0_f64 * t1901 * t56418 * t152717 - t1901 * t10703 * t33978 * t4260 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t15229 * t152815 - 4.0_f64 / 9.0_f64 * t1901 * t112680 * t28760 + 2.0_f64 / 27.0_f64 * t1901 * t15290 * t152888 + 4.0_f64 / 9.0_f64 * t143604 + 4.0_f64 / 9.0_f64 * t143606 - 2.0_f64 / 27.0_f64 * t143608 - t143610 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t24886 * t29150 + 4.0_f64 / 9.0_f64 * t11593 * t10703 * t7629 * t3746 - 4.0_f64 / 9.0_f64 * t1901 * t15312 * t36068 * t684 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t29055 * t28854 - 4.0_f64 * t1901 * t29071 * t6273 * t28501 - 2.0_f64 / 9.0_f64 * t11593 * t2881 * t34207 * t3746 - 4.0_f64 / 3.0_f64 * t1901 * t112920 * t29123 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t29055 * t28847 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t29055 * t28930 - 2.0_f64 / 9.0_f64 * t1901 * t44030 * t36164 + 4.0_f64 / 9.0_f64 * t11593 * t10492 * t34081 * t3746;
    (t153715, t153717, t153720, t153723, t153725, t153788)
}
