//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1333/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1333<F: Float>(t25304: F, t25949: F, t25946: F, t25878: F, t94661: F, t7246: F, t9692: F, t1444: F, t25884: F, t25924: F, t25930: F, t25931: F, t4056: F, t543: F, t7274: F, t7295: F, t7298: F, t7301: F, t94610: F, t94749: F, t94752: F, t94756: F, t94758: F, t94761: F, t94766: F, t94769: F, t94772: F, t94774: F) -> F {
    let t94776 = t25304 * t25949;
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    let t94784 = F::new(0.30356481678079769392e-1) * t7246 * t9692;
    let t94794 = -F::new(0.58544643236296698113e-1) * t94749 - F::new(0.26020884564615598386e1) * t25930 * t25931 * t94752 - F::new(0.28912093960683998208e-1) * t94756 + F::new(0.21951497276451705329e-1) * t94758 - t94761 - F::new(0.77108554593144223218e-1) * t94766 + F::new(0.43368140941025997312e-1) * t94769 - F::new(0.10281140612419229763e-1) * t94772 - F::new(0.77108554593144223218e-1) * t94774 - F::new(0.68549505033305214441e-2) * t94777 - F::new(0.10281140612419229762e0) * t94779 + F::new(0.26020884564615598386e1) * t94610 * t7298 + t94784 - F::new(0.78062653693846795158e1) * t7295 * t25924 * t25884 * t1444 + F::new(0.13010442282307799193e1) * t7295 * t7301 * t7274 * t4056 * t543;
    t94794
}
