//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 986/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk986<F: Float>(t218: F, t219: F, t7992: F, t1174: F, t6149: F, t2204: F, t6165: F, t6175: F, t6177: F, t7970: F, t7973: F, t7975: F, t7980: F, t7983: F, t7986: F, t7990: F) -> (F, F, F, F, F, F) {
    let t7994 = t218 * t219 * t7992;
    let t7996 = t6149 * t1174;
    let t7997 = t7996 * t2204;
    let t7999 = t6165 * t1174;
    let t8000 = t7999 * t2204;
    let t8002 = -0.9494625e0 * t7970 + 0.3071625e0 * t7973 + 0.15358125e0 * t7975 - t6175 + 0.54771111111111111111e0 * t6177 - t7980 - t7983 + 0.24647e0 * t7986 + 0.49294e0 * t7990 + 0.24647e0 * t7994 + 0.142419375e1 * t7997 - 0.76790625e-1 * t8000;
    (t7994, t7996, t7997, t7999, t8000, t8002)
}
