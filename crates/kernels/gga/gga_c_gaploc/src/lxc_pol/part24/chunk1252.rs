//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1252/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1252<F: Float>(t6607: F, t8411: F, t34411: F, t6710: F, t6711: F, t10396: F, t21133: F, t10140: F, t4614: F, t597: F, t10359: F, t4953: F, t10364: F, t1562: F, t1445: F, t31711: F) -> (F, F, F, F, F, F, F) {
    let t34983 = 0.21450293971110256002e1 * t8411 * t6607;
    let t34986 = 0.11502877786176224903e2 * t6710 * t6711 * t34411;
    let t34991 = 0.1853729108614466568e0 * t21133 * t10396;
    let t34994 = 0.30674340763136599742e2 * t597 * t4614 * t10140;
    let t34996 = 0.18404604457881959845e2 * t4953 * t10359;
    let t34999 = 0.18404604457881959845e2 * t1562 * t4614 * t10364;
    let t35021 = 0.23005755572352449806e2 * t597 * t1445 * t31711;
    (t34983, t34986, t34991, t34994, t34996, t34999, t35021)
}
