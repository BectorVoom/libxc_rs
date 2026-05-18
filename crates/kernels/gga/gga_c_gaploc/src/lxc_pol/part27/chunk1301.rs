//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1301/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1301<F: Float>(t10370: F, t4614: F, t574: F, t2859: F, t30949: F, t10447: F, t1562: F, t10324: F, t1641: F, t10365: F, t4953: F, t1445: F, t26428: F, t874: F) -> (F, F, F, F, F, F) {
    let t34181 = F::new(0.12269736305254639897e2) * t574 * t4614 * t10370;
    let t34186 = F::new(0.14300195980740170668e1) * t2859 * t30949;
    let t34189 = F::new(0.18404604457881959845e2) * t1562 * t4614 * t10447;
    let t34191 = F::new(0.12269736305254639897e2) * t1641 * t10324;
    let t34216 = F::new(0.13803453343411469884e2) * t4953 * t10365;
    let t34220 = F::new(0.69017266717057349418e1) * t1562 * t1445 * t26428 * t874;
    (t34181, t34186, t34189, t34191, t34216, t34220)
}
