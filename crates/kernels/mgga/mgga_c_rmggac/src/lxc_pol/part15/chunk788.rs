//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 788/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk788<F: Float>(t7487: F, t9726: F, t2019: F, t2020: F, t9754: F, t2010: F, t2012: F, t5960: F, t1704: F, t236: F, t495: F, t7230: F, t9188: F, t14237: F, t16503: F, t559: F, t8420: F) -> (F, F, F, F, F) {
    let t44854 = t7487 * t9726;
    let t44857 = t2019 * t2020 * t9754;
    let t44860 = t2010 * t2012 * t5960;
    let t44866 = t7230 * t9188 * t236 * t1704 * t495;
    let t44874 = t16503 * t14237 * t559 * t8420;
    (t44854, t44857, t44860, t44866, t44874)
}
