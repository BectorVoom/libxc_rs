//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1392/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1392<F: Float>(t10043: F, t111507: F, t111509: F, t111512: F, t111515: F, t111518: F, t111564: F, t15825: F, t1629: F, t2356: F, t2819: F, t32879: F, t33320: F, t34656: F, t4574: F, t564: F, t6651: F, t9776: F) -> (F,) {
    let t118617 = -t564 * t15825 * t2819 / 16.0 + t111564 - t111507 + t111509 + t111512 - t564 * t1629 * t34656 / 8.0 - t111515 + t111518 + t2356 * t33320 / 16.0 - t564 * t4574 * t10043 / 16.0 - t564 * t6651 * t9776 / 8.0 + t2356 * t32879 / 16.0;
    (t118617,)
}
