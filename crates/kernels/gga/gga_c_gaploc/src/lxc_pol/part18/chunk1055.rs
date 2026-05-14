//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1055/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1055<F: Float>(t10473: F, t1529: F, t2268: F, t31501: F, t550: F, t23726: F, t3347: F, t10113: F, t6313: F, t23609: F, t10132: F, t3355: F, t10249: F, t6305: F, t31585: F, t426: F, t535: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31652 = 0.42682509953514224398e0 * t2268 * t1529 * t10473;
    let t31655 = t550 * t31501;
    let t31660 = 0.2276400530854091968e0 * t23726 * t3347;
    let t31662 = 0.7588001769513639893e-1 * t6313 * t10113;
    let t31672 = 0.37940008847568199467e-1 * t23609 * t3347;
    let t31674 = 0.2276400530854091968e0 * t6313 * t10132;
    let t31679 = 0.25293339231712132977e-1 * t23609 * t3355;
    let t31681 = 0.68292015925622759036e0 * t6305 * t10249;
    let t31685 = 0.56910013271352299198e-1 * t2268 * t535 * t31585 * t426;
    (t31652, t31655, t31660, t31662, t31672, t31674, t31679, t31681, t31685)
}
