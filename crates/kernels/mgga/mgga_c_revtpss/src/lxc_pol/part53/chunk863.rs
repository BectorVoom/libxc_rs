//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 863/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk863<F: Float>(t2457: F, t25945: F, t25944: F, t1426: F, t25920: F, t7063: F, t7286: F, t2470: F, t7285: F, t7289: F, t3974: F, t7259: F) -> (F, F, F, F, F, F, F) {
    let t25946 = t25945 * t2457;
    let t25948 = F::new(0.17135234354032049604e-2) * t25944 * t25946;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25951 = t25950 * t7286;
    let t25953 = t7285 * t2470;
    let t25955 = F::new(0.17135234354032049604e-1) * t7289 * t25953;
    let t25969 = t7259 * t3974;
    (t25946, t25948, t25949, t25951, t25953, t25955, t25969)
}
