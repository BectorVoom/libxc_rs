//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 871/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk871<F: Float>(t19655: F, t358: F, t387: F, t382: F, t1195: F, t6723: F, t1187: F, t19593: F, t5181: F, t3437: F, t19735: F, t3438: F, t1809: F, t5086: F, t19763: F, t20157: F, t20160: F, t20162: F, t20165: F, t20167: F, t20170: F, t20174: F, t20176: F, t20179: F, t20181: F, t20183: F, t20186: F, t20188: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20190 = t358 * t19655;
    let t20191 = t387 * t20190;
    let t20192 = t382 * t20191;
    let t20194 = t1195 * t6723;
    let t20195 = t1187 * t20194;
    let t20197 = t5181 * t19593;
    let t20198 = t3437 * t20197;
    let t20200 = t3438 * t19735;
    let t20201 = t3437 * t20200;
    let t20203 = t1809 * t5086;
    let t20205 = t3438 * t19763;
    let t20206 = t3437 * t20205;
    let t20208 = -t20157 / 16.0 + t20160 / 4.0 + t20162 / 96.0 + t20165 / 6.0 + t20167 / 8.0 + t20170 / 288.0 + t20174 / 256.0 - t20176 / 192.0 - t20179 / 24.0 + t20181 / 24.0 - t20183 / 8.0 + t20186 / 27.0 - t20188 / 192.0 - t20192 / 192.0 - t20195 / 48.0 + t20198 / 576.0 + t20201 / 192.0 + t20203 / 18.0 + t20206 / 192.0;
    (t20191, t20192, t20194, t20195, t20197, t20198, t20200, t20201, t20203, t20205, t20206, t20208)
}
