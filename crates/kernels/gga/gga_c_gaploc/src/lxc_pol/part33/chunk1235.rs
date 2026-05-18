//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1235/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1235<F: Float>(t28099: F, t24390: F, t955: F, t7366: F, t8775: F, t11057: F, t1991: F, t28141: F, t1445: F, t32210: F, t833: F, t28156: F) -> (F, F, F, F, F, F, F) {
    let t32911 = F::new(0.15976219147466979032e-1) * t28099;
    let t32923 = F::new(0.47667319935800568892e0) * t955 * t24390;
    let t32925 = F::new(0.23833659967900284446e0) * t8775 * t7366;
    let t32926 = t1991 * t11057;
    let t32927 = F::new(0.1022478025437886658e1) * t32926;
    let t32928 = F::new(0.31952438294933958064e-1) * t28141;
    let t32931 = F::new(0.11502877786176224903e2) * t833 * t1445 * t32210;
    let t32935 = F::new(0.31952438294933958064e-1) * t28156;
    (t32911, t32923, t32925, t32927, t32928, t32931, t32935)
}
