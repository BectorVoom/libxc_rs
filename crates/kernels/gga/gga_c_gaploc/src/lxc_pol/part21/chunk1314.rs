//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1314/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1314<F: Float>(t34411: F, t6963: F, t6964: F, t30542: F, t30546: F, t21414: F, t26773: F, t3396: F, t4625: F, t27071: F, t544: F, t9287: F) -> (F, F, F, F, F, F) {
    let t34414 = F::new(0.71500979903700853338e0) * t6963 * t6964 * t34411;
    let t34415 = F::new(0.31952438294933958064e0) * t30542;
    let t34416 = F::new(0.12780975317973583226e0) * t30546;
    let t34417 = t26773 * t21414;
    let t34418 = F::new(0.29792074959875355558e-1) * t34417;
    let t34419 = t4625 * t3396;
    let t34420 = F::new(0.19171462976960374838e0) * t34419;
    let t34422 = t544 * t27071 * t9287;
    (t34414, t34415, t34416, t34418, t34420, t34422)
}
