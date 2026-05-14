//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1378/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1378<F: Float>(t292: F, t127089: F, t127124: F, t127173: F, t127209: F, t127244: F, t127297: F, t127337: F, t127382: F, t127424: F, t127469: F, t127515: F, t127567: F, t127609: F, t127647: F, t127688: F, t127723: F) -> (F,) {
    let t293 = 0.1e-59 < t292;
    let t127728 = piecewise3(t293, t127089 + t127124 + t127173 + t127209 + t127244 + t127297 + t127337 + t127382 + t127424 + t127469 + t127515 + t127567 + t127609 + t127647 + t127688 + t127723, 0.0);
    (t127728,)
}
