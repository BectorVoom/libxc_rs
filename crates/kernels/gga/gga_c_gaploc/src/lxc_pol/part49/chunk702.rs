//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 702/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk702<F: Float>(t12079: F, t901: F, t2366: F, t3689: F, t2365: F, t1429: F, t12533: F, t12536: F, t12065: F, t895: F, t11986: F, t874: F, t1445: F, t574: F, t12539: F, t12941: F, t12944: F, t12946: F, t12948: F, t12952: F, t12955: F, t12959: F) -> (F, F, F, F, F) {
    let t13789 = t12079 * t901;
    let t13791 = t2366 * t3689;
    let t13792 = t2365 * t13791;
    let t13793 = t1429 * t13792;
    let t13795 = 0.38342925953920749677e0 * t12533;
    let t13796 = 0.38342925953920749677e0 * t12536;
    let t13798 = t895 * t12065;
    let t13800 = t11986 * t874;
    let t13801 = t1445 * t13800;
    let t13802 = t574 * t13801;
    let t13804 = -t12941 - 0.14896037479937677779e-1 * t12944 + 0.14896037479937677779e-1 * t12946 + 0.14896037479937677779e-1 * t13789 - 0.14896037479937677779e-1 * t13793 - t12948 + t13795 - t13796 - 0.76685851907841499354e0 * t12539 + t12952 - t12955 - t12959 + 0.35750489951850426669e0 * t13798 - 0.46011511144704899612e1 * t13802;
    (t13791, t13792, t13800, t13801, t13804)
}
