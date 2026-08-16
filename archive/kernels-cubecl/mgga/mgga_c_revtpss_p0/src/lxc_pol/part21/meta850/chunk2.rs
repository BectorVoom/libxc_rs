//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3194/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3194<F: Float>(t10356: F, t12256: F, t12268: F, t12787: F, t12789: F, t12832: F, t12910: F, t12922: F, t12926: F, t12933: F, t17569: F, t17605: F, t17710: F, t17724: F, t3625: F, t3720: F, t44225: F, t44551: F, t44578: F, t44609: F, t44696: F, t44952: F, t45371: F, t471: F, t5332: F, t5346: F, t5351: F, t5381: F, t58777: F, t58780: F, t58785: F, t58791: F, t58793: F, t58798: F, t58804: F, t58824: F, t58827: F, t58831: F) -> F {
    let t58842 = -F::cast_from(0.42344709252414555035e-4_f64) * t58777 - F::cast_from(0.15879265969655458138e-3_f64) * t44696 + F::cast_from(0.38586616306262763275e-2_f64) * t44578 * t3720 * t17710 * t58780 - F::cast_from(0.64311027177104605458e-3_f64) * t45371 * t3720 * t17710 * t58785 + F::cast_from(0.85748036236139473944e-3_f64) * t58791 - F::cast_from(0.12862205435420921092e-2_f64) * t44952 * t3720 * t5332 * t58793 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t5351 * t58798 + F::cast_from(0.25724410870841842183e-2_f64) * t44551 * t3720 * t5332 * t58804 - F::cast_from(0.63517063878621832552e-3_f64) * t3625 * t44225 * t5351 * t471 * t12256 * t10356 - F::cast_from(0.12862205435420921092e-2_f64) * t12832 * t17724 - F::cast_from(0.3811023832717309953e-2_f64) * t17605 * t12789 - F::cast_from(0.85748036236139473944e-3_f64) * t5381 * t12926 + F::cast_from(0.85748036236139473944e-3_f64) * t17569 * t12922 + F::cast_from(0.63517063878621832551e-4_f64) * t58824 - F::cast_from(0.11433071498151929859e-2_f64) * t58827 + F::cast_from(0.42874018118069736972e-3_f64) * t17569 * t12933 - F::cast_from(0.38586616306262763275e-2_f64) * t44609 * t3720 * t5346 * t58831 + F::cast_from(0.14291339372689912324e-2_f64) * t3625 * t12787 * t5351 * t471 * t12268 * t10356;
    t58842
}
