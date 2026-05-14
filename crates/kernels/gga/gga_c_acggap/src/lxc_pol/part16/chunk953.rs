//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 953/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk953<F: Float>(t368: F, t5659: F, t7380: F, t1795: F, t355: F, t1083: F, t2095: F, t7839: F, t9593: F, t1165: F, t2068: F, t38837: F, t8600: F, t30226: F, t30230: F, t30233: F, t30239: F, t30240: F, t30243: F, t30247: F, t30249: F, t33963: F, t33983: F, t33987: F, t33995: F, t36876: F, t36889: F) -> (F, F, F, F) {
    let t38889 = t368 * t5659;
    let t38890 = t7380 * t38889;
    let t38892 = t355 * t1795;
    let t38893 = t1083 * t38892;
    let t38894 = t2095 * t38893;
    let t38899 = t7839 * t9593;
    let t38903 = t2068 * t1165 * t8600 * t38837;
    let t38905 = 0.22921875e-1 * t38890 + 0.1528125e-1 * t38894 - t36876 + t33963 + 0.85748036236139473944e-3 * t30226 + t30230 + t30233 + t30239 + 0.10718504529517434243e-3 * t30240 + t30243 - t30247 - 0.45351183609335988444e-1 * t30249 + 0.42874018118069736972e-3 * t38899 - t33983 + t36889 + t33987 + 0.18868855373762491241e-2 * t38903 + t33995;
    (t38889, t38892, t38893, t38905)
}
