//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1184/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1184<F: Float>(t14626: F, t3447: F, t833: F, t2718: F, t8556: F, t11039: F, t2194: F, t1445: F, t2530: F, t813: F, t8528: F, t2949: F, t7112: F, t3492: F, t6024: F, t16239: F, t3477: F) -> (F, F, F, F, F, F, F) {
    let t33905 = 0.51123901271894332903e1 * t833 * t14626 * t3447;
    let t33907 = 0.47667319935800568892e0 * t2718 * t8556;
    let t33912 = 0.92023022289409799224e1 * t2194 * t11039;
    let t33916 = 0.92023022289409799224e1 * t813 * t1445 * t8528 * t2530;
    let t33920 = 0.46011511144704899612e1 * t813 * t1445 * t2949 * t7112;
    let t33922 = 0.11502877786176224903e2 * t6024 * t3492;
    let t33927 = 0.71500979903700853338e0 * t16239 * t3477;
    (t33905, t33907, t33912, t33916, t33920, t33922, t33927)
}
