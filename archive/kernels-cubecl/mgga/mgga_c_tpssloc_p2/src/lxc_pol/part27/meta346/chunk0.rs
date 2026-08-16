//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1433/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1433<F: Float>(t12680: F, t607: F, t2250: F, t3981: F, t12606: F, t43: F, t1409: F, t2244: F, t9300: F, t2274: F, t3966: F, t3990: F) -> (F, F, F, F, F, F) {
    let t12681 = t12680 * t607;
    let t12684 = t3981 * t2250;
    let t12687 = t43 * t12606;
    let t12695 = t9300 * t1409 * t2244;
    let t12698 = t2274 * t3966;
    let t12699 = t12698 * t607;
    let t12702 = t3990 * t2250;
    (t12681, t12684, t12687, t12695, t12699, t12702)
}
