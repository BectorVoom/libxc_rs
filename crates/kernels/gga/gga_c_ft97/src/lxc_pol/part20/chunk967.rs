//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 967/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk967<F: Float>(t18986: F, t2: F, t4: F, t26: F, t6213: F, t6963: F, t10697: F, t1218: F, t1466: F, t1479: F, t25463: F, t25487: F, t25489: F, t25492: F, t29030: F, t29035: F, t29042: F, t29045: F, t29047: F, t29410: F, t301: F, t6210: F, t6391: F, t7024: F, t7129: F, t830: F) -> (F, F, F) {
    let t29414 = t18986 * t2;
    let t29415 = t29414 * t4;
    let t29416 = t29415 * t26;
    let t29419 = t6963 * t6213;
    let t29422 = -2.0 * t29030 + t25463 / 54.0 - t1466 * t29035 / 3.0 + t6210 * t7024 / 6.0 + t1466 * t29042 / 6.0 - 2.0 * t29045 - 12.0 * t10697 * t29047 - t830 * t7129 + t25487 - t301 * t29410 - t25489 / 18.0 - t25492 / 18.0 + t29416 * t1479 / 6.0 - t29419 / 18.0 - t1218 * t6391;
    (t29415, t29416, t29422)
}
