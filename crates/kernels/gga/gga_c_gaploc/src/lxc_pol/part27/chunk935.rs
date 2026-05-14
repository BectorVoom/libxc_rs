//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 935/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk935<F: Float>(t12223: F, t723: F, t1445: F, t12214: F, t12213: F, t701: F, t10927: F, t10934: F, t10935: F, t10937: F, t12207: F, t12210: F, t12215: F, t12220: F, t1998: F, t2009: F, t2087: F, t2103: F, t780: F, t813: F, t833: F, t9935: F, t9937: F, t9942: F, t9946: F) -> (F, F, F, F, F, F) {
    let t12224 = t12223 * t723;
    let t12225 = t1445 * t12224;
    let t12228 = t1445 * t12214;
    let t12231 = t12213 * t701;
    let t12232 = t1445 * t12231;
    let t12235 = -t10927 + t10934 + 0.35750489951850426669e0 * t780 * t12207 - 0.35750489951850426669e0 * t12210 * t2009 + 0.71500979903700853338e0 * t2103 * t12215 - 0.69017266717057349418e1 * t2087 * t12220 - 0.46011511144704899612e1 * t813 * t12225 + 0.11502877786176224903e2 * t833 * t12228 - 0.23005755572352449806e1 * t1998 * t12232 + t9935 + t9937 - t9942 - t9946 + t10935 + t10937;
    (t12224, t12225, t12228, t12231, t12232, t12235)
}
