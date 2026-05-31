//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1464/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1464<F: Float>(t224: F, t38891: F, t38897: F, t38906: F, t39520: F, t12326: F, t617: F, t1676: F, t33952: F, t33966: F, t33968: F, t33974: F, t33979: F, t33997: F, t34008: F, t34012: F, t34018: F, t34023: F, t35240: F, t3751: F, t38876: F, t38880: F, t38881: F, t39339: F, t39342: F, t39519: F) -> F {
    let t39523 = t224 * (t38891 + t38897 + t38906 + t39520);
    let t39524 = t617 * t12326;
    let t39526 = t1676 * t3751 + t33952 + t33966 - t33968 - t33974 - t33979 + t33997 + t34008 + t34012 - t34018 + t34023 - t35240 - t38876 + t38880 - t38881 + t39339 - t39342 - t39519 + t39523 + F::cast_from(2.0_f64) * t39524;
    t39526
}
