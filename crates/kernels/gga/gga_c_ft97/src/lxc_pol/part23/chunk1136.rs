//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1136/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1136<F: Float>(t1403: F, t2399: F, t6753: F, t24220: F, t6745: F, t27964: F, t681: F, t24237: F, t28020: F, t27929: F, t5996: F, t27997: F, t2567: F, t6907: F, t27993: F, t27975: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109556 = t1403 * t2399 * t6753;
    let t109577 = t6745 * t24220 / 9.0;
    let t109589 = 2.0 / 9.0 * t1403 * t681 * t27964;
    let t109597 = t24237 * t28020 / 27.0;
    let t109634 = t5996 * t27929 / 9.0;
    let t109643 = 2.0 / 3.0 * t24237 * t27997;
    let t109652 = t6907 * t2567;
    let t109670 = t24237 * t27993 / 27.0;
    let t109700 = 2.0 / 9.0 * t1403 * t681 * t27975;
    (t109556, t109577, t109589, t109597, t109634, t109643, t109652, t109670, t109700)
}
