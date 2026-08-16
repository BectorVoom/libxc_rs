//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 658/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk658(t10789: f64, t313: f64, t3503: f64, t4614: f64, t2087: f64, t10677: f64, t723: f64, t1445: f64, t10783: f64, t3447: f64, t833: f64, t3483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10948 = t313 * t10789;
    let t10951 = t4614 * t3503;
    let t10953 = 0.92023022289409799224e1_f64 * t2087 * t10951;
    let t10954 = t10677 * t723;
    let t10955 = t1445 * t10954;
    let t10958 = t1445 * t10783;
    let t10961 = t4614 * t3447;
    let t10963 = 0.15337170381568299871e2_f64 * t833 * t10961;
    let t10964 = t4614 * t3483;
    (t10948, t10953, t10955, t10958, t10963, t10964)
}
