//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1287/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1287<F: Float>(t21787: F, t2922: F, t9292: F, t5974: F, t9273: F, t2899: F, t9310: F, t774: F, t9315: F, t7736: F, t9320: F, t7742: F, t9324: F, t17766: F, t17782: F, t2009: F, t2104: F, t2105: F, t21669: F, t25136: F, t2900: F, t302: F, t3515: F, t3679: F, t5984: F, t761: F, t7725: F, t9274: F, t9279: F, t9282: F, t9284: F, t9559: F) -> (F,) {
    let t25434 = t2922 * t21787 * t9292;
    let t25448 = t2922 * t5974 * t9273;
    let t25453 = t2899 * t5974 * t9310;
    let t25456 = t2899 * t774 * t9315;
    let t25459 = t7736 * t774 * t9320;
    let t25462 = t7742 * t774 * t9324;
    let t25476 = 0.11433071498151929859e-2 * t25434 - 0.42874018118069736972e-3 * t2104 * t2105 * t3515 * t2009 * t761 + 0.45732285992607719436e-2 * t7725 * t9279 - 0.22866142996303859718e-2 * t21669 * t9284 - 0.45732285992607719436e-2 * t7725 * t9274 + 0.57165357490759649296e-3 * t25448 + 0.45732285992607719436e-2 * t5984 * t9559 - 0.11433071498151929859e-2 * t25453 + 0.11433071498151929859e-2 * t25456 + 0.17149607247227894789e-2 * t25459 - 0.17149607247227894789e-2 * t25462 - 0.85748036236139473944e-3 * t2899 * t2105 * t3679 * t17782 + 0.85748036236139473944e-3 * t2899 * t302 * t2900 * t25136 + 0.12862205435420921092e-2 * t7736 * t302 * t9282 * t17766;
    (t25476,)
}
