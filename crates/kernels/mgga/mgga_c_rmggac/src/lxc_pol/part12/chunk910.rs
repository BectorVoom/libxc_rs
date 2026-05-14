//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 910/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk910<F: Float>(t2019: F, t2020: F, t8858: F, t2010: F, t2012: F, t5757: F, t4962: F, t8854: F, t5002: F, t8850: F, t1652: F, t1971: F, t495: F, t515: F, t7230: F, t34944: F, t40888: F) -> (F, F, F, F, F, F, F, F) {
    let t41604 = t2019 * t2020 * t8858;
    let t41605 = 0.30487649791575028314e-3 * t41604;
    let t41607 = t2010 * t2012 * t5757;
    let t41610 = t2010 * t2012 * t4962;
    let t41613 = t2019 * t2020 * t8854;
    let t41614 = 0.30487649791575028314e-3 * t41613;
    let t41616 = t2010 * t2012 * t5002;
    let t41619 = t2019 * t2020 * t8850;
    let t41620 = 0.30487649791575028314e-3 * t41619;
    let t41627 = t7230 * t1971 * t515 * t1652 * t495;
    let t41631 = t34944 * t40888;
    (t41605, t41607, t41610, t41614, t41616, t41620, t41627, t41631)
}
