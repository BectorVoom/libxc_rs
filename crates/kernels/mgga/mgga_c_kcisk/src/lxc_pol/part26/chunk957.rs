//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 957/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk957<F: Float>(t1309: F, t1324: F, t20036: F, t20084: F, t20097: F, t20255: F, t2164: F, t2170: F, t25981: F, t25985: F, t25994: F, t25999: F, t26003: F, t26008: F, t26017: F, t26020: F, t26023: F, t3935: F, t3966: F, t6157: F, t6189: F, t6201: F, t6207: F, t6213: F, t8033: F, t8041: F, t8045: F, t8050: F) -> (F,) {
    let t26026 = 0.71963154864709268852e-1 * t6157 * t6189 + 0.23987718288236422951e-1 * t3966 * t8033 + 0.59969295720591057377e-2 * t25981 - 0.95950873152945691803e-1 * t20084 * t2164 + 0.11993859144118211475e-1 * t25985 + 0.21588946459412780656e0 * t6157 * t6207 - 0.35981577432354634426e-1 * t3966 * t8041 + 0.17990788716177317213e-1 * t3966 * t8045 + 0.17990788716177317213e-1 * t1309 * t25994 - 0.32383419689119170984e0 * t1309 * t25999 + 0.21588946459412780656e0 * t1309 * t26003 - t20036 - 0.35981577432354634427e-1 * t20255 * t6201 - 0.5397236614853195164e-1 * t26008 * t1324 + 0.10794473229706390328e0 * t3966 * t8050 - 0.10794473229706390328e0 * t20097 * t2170 - 0.10794473229706390328e0 * t6157 * t6213 - 0.1439263097294185377e0 * t3935 * t26017 + 0.55971342672551653552e-1 * t3935 * t26020 + 0.95950873152945691804e-1 * t3935 * t26023;
    (t26026,)
}
