//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1005/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1005<F: Float>(t127368: F, t32110: F, t4248: F, t2322: F, t33581: F, t4254: F, t5517: F, t651: F, t8460: F, t1868: F, t7311: F, t25082: F, t8717: F, t27833: F, t8600: F, t1916: F, t32369: F) -> (F, F, F, F, F, F, F, F) {
    let t127369 = 2.0 * t127368;
    let t127370 = t4248 * t32110;
    let t127371 = 2.0 * t127370;
    let t127372 = t2322 * t33581;
    let t127373 = 2.0 * t127372;
    let t127374 = t4254 * t33581;
    let t127375 = 2.0 * t127374;
    let t127377 = t651 * t5517 * t8460;
    let t127378 = 2.0 * t127377;
    let t127381 = t1868 * t7311;
    let t127384 = 6.0 * t25082 * t8717 * t127381;
    let t127385 = t27833 * t8600;
    let t127442 = 12.0 * t1916 * t32369;
    (t127369, t127371, t127373, t127375, t127378, t127384, t127385, t127442)
}
