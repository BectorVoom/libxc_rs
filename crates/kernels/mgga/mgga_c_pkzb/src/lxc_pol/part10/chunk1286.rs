//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1286/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1286<F: Float>(t25121: F, t301: F, t2027: F, t3515: F, t1885: F, t2026: F, t2030: F, t2031: F, t2039: F, t2104: F, t2105: F, t21643: F, t21651: F, t21655: F, t21657: F, t21661: F, t21667: F, t21729: F, t25363: F, t2899: F, t2922: F, t302: F, t3679: F, t3685: F, t5693: F, t758: F, t7640: F, t7658: F, t7664: F, t7666: F, t7707: F, t7725: F, t9282: F, t9293: F, t9564: F) -> (F, F) {
    let t25391 = t301 * t25121;
    let t25401 = t3515 * t2027;
    let t25431 = 0.57165357490759649296e-3 * t21643 + 0.85748036236139473944e-3 * t2026 * t758 * t25391 * t2031 - 0.19055119163586549765e-3 * t21651 - 0.19309187419101037096e-1 * t21655 - 0.20325460441158986416e-2 * t21657 - 0.2540682555144873302e-3 * t21661 - 0.57165357490759649296e-3 * t21667 - 0.85748036236139473944e-3 * t2899 * t2105 * t25401 * t2031 + 0.42874018118069736972e-3 * t2922 * t2105 * t25401 * t2039 - 0.45732285992607719436e-2 * t7707 * t9564 + 0.12862205435420921092e-2 * t2104 * t5693 * t3685 * t7640 + 0.25724410870841842183e-2 * t2899 * t5693 * t3679 * t2030 * t1885 + 0.21437009059034868486e-3 * t7664 * t302 * t9282 * t7658 - 0.21437009059034868486e-3 * t21729 * t302 * t25363 * t7666 - 0.91464571985215438874e-2 * t7725 * t9293;
    (t25391, t25431)
}
