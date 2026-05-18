//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 658/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk658<F: Float>(t10789: F, t313: F, t3503: F, t4614: F, t2087: F, t10677: F, t723: F, t1445: F, t10783: F, t3447: F, t833: F, t3483: F) -> (F, F, F, F, F, F) {
    let t10948 = t313 * t10789;
    let t10951 = t4614 * t3503;
    let t10953 = F::new(0.92023022289409799224e1) * t2087 * t10951;
    let t10954 = t10677 * t723;
    let t10955 = t1445 * t10954;
    let t10958 = t1445 * t10783;
    let t10961 = t4614 * t3447;
    let t10963 = F::new(0.15337170381568299871e2) * t833 * t10961;
    let t10964 = t4614 * t3483;
    (t10948, t10953, t10955, t10958, t10963, t10964)
}
