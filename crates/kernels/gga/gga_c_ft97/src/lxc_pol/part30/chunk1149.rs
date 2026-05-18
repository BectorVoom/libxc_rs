//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1149/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1149<F: Float>(t34012: F, t4299: F, t10688: F, t36068: F, t2843: F, t6386: F, t7124: F, t36042: F, t875: F, t34333: F, t6963: F, t10492: F, t10703: F, t112680: F, t112920: F, t11593: F, t143604: F, t143606: F, t143608: F, t143610: F, t15229: F, t152717: F, t152815: F, t152888: F, t15290: F, t15312: F, t15460: F, t1901: F, t24886: F, t28501: F, t28760: F, t2881: F, t28847: F, t28854: F, t28930: F, t29055: F, t29071: F, t29123: F, t29150: F, t33978: F, t34081: F, t34207: F, t36164: F, t3746: F, t4260: F, t44030: F, t56418: F, t6273: F, t684: F, t7629: F) -> (F, F, F, F, F, F) {
    let t153715 = t34012 * t4299;
    let t153717 = t10688 * t36068;
    let t153720 = t2843 * t6386 * t7124;
    let t153723 = t2843 * t36042 * t875;
    let t153725 = t6963 * t34333;
    let t153788 = F::new(2.0) / F::new(3.0) * t1901 * t56418 * t152717 - t1901 * t10703 * t33978 * t4260 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t15229 * t152815 - F::new(4.0) / F::new(9.0) * t1901 * t112680 * t28760 + F::new(2.0) / F::new(27.0) * t1901 * t15290 * t152888 + F::new(4.0) / F::new(9.0) * t143604 + F::new(4.0) / F::new(9.0) * t143606 - F::new(2.0) / F::new(27.0) * t143608 - t143610 / F::new(27.0) + F::new(2.0) / F::new(9.0) * t1901 * t24886 * t29150 + F::new(4.0) / F::new(9.0) * t11593 * t10703 * t7629 * t3746 - F::new(4.0) / F::new(9.0) * t1901 * t15312 * t36068 * t684 - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t29055 * t28854 - F::new(4.0) * t1901 * t29071 * t6273 * t28501 - F::new(2.0) / F::new(9.0) * t11593 * t2881 * t34207 * t3746 - F::new(4.0) / F::new(3.0) * t1901 * t112920 * t29123 - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t29055 * t28847 - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t29055 * t28930 - F::new(2.0) / F::new(9.0) * t1901 * t44030 * t36164 + F::new(4.0) / F::new(9.0) * t11593 * t10492 * t34081 * t3746;
    (t153715, t153717, t153720, t153723, t153725, t153788)
}
