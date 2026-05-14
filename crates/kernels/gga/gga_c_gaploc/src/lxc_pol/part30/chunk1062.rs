//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1062/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1062<F: Float>(t1367: F, t31543: F, t196: F, t21488: F, t555: F, t2787: F, t6509: F, t590: F, t1570: F, t10177: F, t4538: F, t189: F, t3394: F, t488: F, t2268: F, t2440: F, t2756: F) -> (F, F, F, F, F, F, F, F) {
    let t31764 = t31543 * t1367;
    let t31766 = 0.44263343655496232709e-1 * t21488 * t196 * t555 * t31764;
    let t31769 = t2787 * t6509;
    let t31770 = t590 * t31769;
    let t31772 = 0.7588001769513639893e-1 * t21488 * t196 * t1570 * t31770;
    let t31775 = t590 * t10177;
    let t31777 = 0.37940008847568199465e-1 * t21488 * t196 * t4538 * t31775;
    let t31783 = 0.63233348079280332442e-2 * t21488 * t196 * t189 * t6509 * t3394 * t488;
    let t31786 = 0.56910013271352299198e-1 * t2268 * t2440 * t2756;
    (t31764, t31766, t31770, t31772, t31775, t31777, t31783, t31786)
}
