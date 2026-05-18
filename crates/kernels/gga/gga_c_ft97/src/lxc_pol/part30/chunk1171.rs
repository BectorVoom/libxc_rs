//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1171/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1171<F: Float>(t1882: F, t36138: F, t36224: F, t36204: F, t114578: F, t126613: F, t144005: F, t144131: F, t144162: F, t144176: F, t144178: F, t144184: F, t144190: F, t15369: F, t153723: F, t15460: F, t1901: F, t29185: F, t29302: F, t29307: F, t296: F, t3281: F, t34102: F, t36042: F, t3746: F, t4176: F, t4246: F, t4256: F, t446: F, t53797: F, t6260: F, t6353: F, t6365: F, t6374: F, t7131: F, t7686: F, t824: F, t835: F, t840: F, t871: F, t99672: F) -> F {
    let t154728 = t1882 * t36138;
    let t154734 = t1882 * t36224;
    let t154738 = t1882 * t36204;
    let t154783 = t144162 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t154728 - F::new(2.0) / F::new(3.0) * t446 * t840 * t7131 * t6260 + t154734 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t144176 + F::new(2.0) / F::new(9.0) * t144178 + F::new(2.0) / F::new(9.0) * t154738 + F::new(2.0) / F::new(3.0) * t446 * t296 * t153723 + F::new(2.0) / F::new(9.0) * t3281 * t835 * t7686 * t3746 + F::new(2.0) / F::new(3.0) * t446 * t840 * t4246 * t34102 + F::new(4.0) / F::new(9.0) * t53797 * t99672 * t29185 - F::new(4.0) / F::new(3.0) * t1901 * t15369 * t114578 * t6365 - F::new(4.0) / F::new(3.0) * t1901 * t15460 * t126613 * t6374 + t1901 * t144005 * t4256 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t1901 * t15460 * t144131 * t4176 + t144184 + t446 * t840 * t871 * t36042 * t824 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t840 * t6353 * t29302 + F::new(2.0) / F::new(3.0) * t446 * t840 * t6353 * t29307 - t144190 / F::new(9.0);
    t154783
}
