//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1144/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1144<F: Float>(t34287: F, t36071: F, t1774: F, t7087: F, t7570: F, t1526: F, t7079: F, t9483: F, t1466: F, t28804: F, t1091: F, t13616: F, t142512: F, t1477: F, t2320: F, t28517: F, t28521: F, t28525: F, t28534: F, t28720: F, t28788: F, t28796: F, t28813: F, t29414: F, t34284: F, t34296: F, t36075: F, t36080: F, t3704: F, t3746: F, t461: F, t6210: F, t6216: F, t6261: F, t666: F, t7150: F, t7571: F) -> F {
    let t153598 = t36071 * t34287;
    let t153611 = t7570 * t1774 * t7087;
    let t153617 = t1526 * t9483 * t7079;
    let t153619 = t1466 * t28804;
    let t153621 = -t142512 / F::new(9.0) - t6216 * t28813 / F::new(9.0) + t6216 * t28525 / F::new(27.0) - t6216 * t28517 / F::new(9.0) - t6216 * t28521 / F::new(9.0) - t29414 * t7150 * t7571 / F::new(6.0) - t7570 * t461 * t28720 / F::new(6.0) - t1466 * t3704 * t1477 * t3746 / F::new(9.0) + t6210 * t36075 / F::new(18.0) + t1466 * t666 * t6261 * t1091 / F::new(18.0) + t153598 / F::new(18.0) + t1526 * t13616 * t28788 / F::new(6.0) - t1526 * t2320 * t28796 / F::new(12.0) - t34284 * t36080 / F::new(6.0) - t36071 * t34296 / F::new(6.0) + t153611 / F::new(18.0) - t1526 * t2320 * t28534 / F::new(12.0) - t153617 / F::new(36.0) - t153619 / F::new(9.0);
    t153621
}
