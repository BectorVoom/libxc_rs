//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1153/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1153<F: Float>(t7114: F, t98724: F, t1882: F, t36228: F, t112883: F, t114820: F, t143653: F, t1476: F, t152861: F, t15294: F, t153372: F, t15369: F, t15460: F, t1901: F, t2347: F, t24898: F, t2749: F, t2766: F, t2862: F, t28924: F, t29137: F, t29369: F, t296: F, t319: F, t34159: F, t35972: F, t36003: F, t36121: F, t36152: F, t3886: F, t4141: F, t4261: F, t4266: F, t446: F, t56098: F, t56815: F, t6386: F, t6393: F, t7021: F, t7036: F, t7662: F, t7679: F, t824: F, t840: F, t871: F, t875: F, t882: F) -> (F, F) {
    let t154017 = t98724 * t7114;
    let t154059 = t1882 * t36228;
    let t154082 = F::new(2.0) / F::new(27.0) * t1901 * t15294 * t7679 * t2347 * t3886 - F::new(2.0) / F::new(9.0) * t1901 * t56098 * t34159 + F::new(4.0) / F::new(3.0) * t446 * t2862 * t6393 * t7036 + F::new(4.0) / F::new(3.0) * t446 * t296 * t154017 + F::new(2.0) / F::new(3.0) * t446 * t840 * t871 * t7021 * t6386 - t446 * t840 * t36003 * t824 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t840 * t871 * t1476 * t28924 - F::new(4.0) / F::new(3.0) * t1901 * t15369 * t24898 * t29369 - F::new(4.0) / F::new(3.0) * t1901 * t56815 * t36152 - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t112883 * t7114 + t1901 * t143653 * t4261 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t1901 * t143653 * t4266 - F::new(2.0) / F::new(27.0) * t1901 * t2766 * t7662 * t4141 - F::new(4.0) / F::new(3.0) * t1901 * t114820 * t29137 + t154059 / F::new(9.0) - t446 * t840 * t882 * t35972 / F::new(3.0) - t446 * t840 * t319 * t153372 / F::new(3.0) + t446 * t840 * t871 * t35972 * t875 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t2862 * t319 * t152861 + t446 * t840 * t2749 * t36121 / F::new(3.0);
    (t154017, t154082)
}
