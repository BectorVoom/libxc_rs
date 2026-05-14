//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 597/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk597<F: Float>(t8528: F, t935: F, t1445: F, t813: F, t3477: F, t5771: F, t10713: F, t1457: F, t2103: F, t10717: F, t10721: F, t3470: F, t8478: F, t8638: F, t10948: F, t10953: F, t10955: F, t10958: F, t10963: F, t10966: F, t10967: F, t10971: F, t2004: F, t2639: F, t833: F) -> (F,) {
    let t10972 = t8528 * t935;
    let t10973 = t1445 * t10972;
    let t10975 = 0.46011511144704899612e1 * t813 * t10973;
    let t10977 = 0.71500979903700853338e0 * t5771 * t3477;
    let t10978 = t1457 * t10713;
    let t10980 = 0.71500979903700853338e0 * t2103 * t10978;
    let t10981 = t1457 * t10717;
    let t10983 = 0.71500979903700853338e0 * t2103 * t10981;
    let t10984 = t1457 * t10721;
    let t10988 = 0.10725146985555128001e1 * t8478 * t3470;
    let t10990 = 0.10725146985555128001e1 * t8638 * t3470;
    let t10991 = -0.10725146985555128001e1 * t10948 * t2639 - t10953 - 0.46011511144704899612e1 * t813 * t10955 + 0.11502877786176224903e2 * t833 * t10958 + t10963 - t10966 + 0.71500979903700853338e0 * t2103 * t10967 - t10971 - t10975 + t10977 + t10980 + t10983 + 0.35750489951850426669e0 * t2004 * t10984 - t10988 - t10990;
    (t10991,)
}
