//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 991/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk991<F: Float>(t2259: F, t26971: F, t2257: F, t7964: F, t7974: F, t27482: F, t2264: F, t4479: F, t1628: F, t7996: F, t2167: F, t4527: F, t1655: F, t7671: F, t7889: F, t8130: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27651 = t26971 * t2259;
    let t27653 = 0.7722800925925925926e-4 * t2257 * t27651;
    let t27654 = t7964 * t7974;
    let t27668 = 0.38691203703703703703e-3 * t27482;
    let t27702 = t2264 * t4479;
    let t27710 = t7996 * t1628;
    let t27733 = t4527 * t2167;
    let t27741 = t1655 * t7671;
    let t28311 = t8130 * t7889;
    (t27651, t27653, t27654, t27668, t27702, t27710, t27733, t27741, t28311)
}
