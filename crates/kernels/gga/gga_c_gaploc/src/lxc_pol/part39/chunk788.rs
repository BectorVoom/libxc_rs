//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 788/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk788<F: Float>(t13800: F, t1445: F, t574: F, t12539: F, t12941: F, t12944: F, t12946: F, t12948: F, t12952: F, t12955: F, t12959: F, t13789: F, t13793: F, t13795: F, t13796: F, t13798: F) -> (F, F) {
    let t13801 = t1445 * t13800;
    let t13802 = t574 * t13801;
    let t13804 = -t12941 - F::new(0.14896037479937677779e-1) * t12944 + F::new(0.14896037479937677779e-1) * t12946 + F::new(0.14896037479937677779e-1) * t13789 - F::new(0.14896037479937677779e-1) * t13793 - t12948 + t13795 - t13796 - F::new(0.76685851907841499354e0) * t12539 + t12952 - t12955 - t12959 + F::new(0.35750489951850426669e0) * t13798 - F::new(0.46011511144704899612e1) * t13802;
    (t13801, t13804)
}
