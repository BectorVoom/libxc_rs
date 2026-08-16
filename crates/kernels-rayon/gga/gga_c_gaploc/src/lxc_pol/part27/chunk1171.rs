//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1171/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1171(t10473: f64, t1529: f64, t2268: f64, t23726: f64, t3347: f64, t10113: f64, t6313: f64, t23609: f64, t10132: f64, t3355: f64, t10249: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31652 = 0.42682509953514224398e0_f64 * t2268 * t1529 * t10473;
    let t31660 = 0.2276400530854091968e0_f64 * t23726 * t3347;
    let t31662 = 0.7588001769513639893e-1_f64 * t6313 * t10113;
    let t31672 = 0.37940008847568199467e-1_f64 * t23609 * t3347;
    let t31674 = 0.2276400530854091968e0_f64 * t6313 * t10132;
    let t31679 = 0.25293339231712132977e-1_f64 * t23609 * t3355;
    let t31681 = 0.68292015925622759036e0_f64 * t6305 * t10249;
    (t31652, t31660, t31662, t31672, t31674, t31679, t31681)
}
