//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 959/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk959<F: Float>(t1008: F, t1013: F, t72: F, t5579: F, t1354: F, t4674: F, t1291: F, t1303: F, t1355: F, t16786: F, t2043: F, t23732: F, t23774: F, t26763: F, t29551: F, t29555: F, t30038: F, t30042: F, t30058: F, t4675: F, t5785: F, t5802: F, t5838: F) -> (F, F, F, F, F) {
    let t30062 = t1008 * t1013;
    let t30063 = t72 * t30062;
    let t30064 = t5579 * t30063;
    let t30067 = t4674 * t1354;
    let t30070 = -0.45306850413028723348e0 * t5802 * t30038 + 0.22653425206514361674e0 * t1355 * t30042 - 0.22653425206514361674e0 * t2043 * t30042 - 0.45306850413028723348e0 * t4675 * t1291 + 0.45306850413028723348e0 * t16786 * t1291 + 0.45306850413028723348e0 * t5785 * t30038 - 0.66678001092592592595e-1 * t26763 - 0.16669500273148148149e-1 * t5838 * t29551 - 0.22226000364197530865e-1 * t5838 * t29555 - 0.30005100491666666667e0 * t23774 * t5579 * t30058 + 0.40006800655555555556e0 * t23732 * t30064 - 0.10001700163888888889e0 * t30067 * t1303;
    (t30062, t30063, t30064, t30067, t30070)
}
